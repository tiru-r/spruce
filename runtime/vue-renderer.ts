import { createRenderer, VNode, RendererOptions } from 'vue'

// Import our Rust runtime bindings
import { VueNativeRuntimeBinding } from './dist/index.js'

interface NativeNode {
  id: number
  type: string
  props: Record<string, any>
  children: NativeNode[]
  parent?: NativeNode
}

interface NativeElement extends NativeNode {
  tag: string
}

interface NativeText extends NativeNode {
  text: string
}

// Initialize Rust runtime
const nativeRuntime = new VueNativeRuntimeBinding()

// Custom renderer options for VueNative-RS
const rendererOptions: RendererOptions<NativeNode, NativeElement> = {
  // Create element (Vue component -> Native view)
  createElement(type: string, isSVG?: boolean, isCustomizedBuiltIn?: string): NativeElement {
    console.log(`Creating native element: ${type}`)
    
    const element: NativeElement = {
      id: generateViewId(),
      type: 'element',
      tag: mapVueTagToNative(type),
      props: {},
      children: []
    }

    return element
  },

  // Create text node  
  createText(text: string): NativeText {
    console.log(`Creating native text: ${text}`)
    
    return {
      id: generateViewId(),
      type: 'text',
      text,
      props: { text },
      children: []
    }
  },

  // Create comment (usually ignored in native)
  createComment(text: string): NativeNode {
    return {
      id: generateViewId(),
      type: 'comment',
      props: { comment: text },
      children: []
    }
  },

  // Set text content
  setText(node: NativeText, text: string): void {
    console.log(`Setting text: ${text}`)
    node.text = text
    node.props.text = text
    
    // Update native view
    nativeRuntime.updateView(node.id, JSON.stringify(node.props))
  },

  // Set element text
  setElementText(el: NativeElement, text: string): void {
    console.log(`Setting element text: ${text}`)
    el.props.text = text
    
    // Update native view
    nativeRuntime.updateView(el.id, JSON.stringify(el.props))
  },

  // Insert node
  insert(child: NativeNode, parent: NativeElement, anchor?: NativeNode | null): void {
    console.log(`Inserting child ${child.id} into parent ${parent.id}`)
    
    child.parent = parent
    
    if (anchor) {
      const index = parent.children.indexOf(anchor)
      parent.children.splice(index, 0, child)
    } else {
      parent.children.push(child)
    }

    // Create native view
    const componentData = {
      type: child.type === 'element' ? (child as NativeElement).tag : 'Text',
      props: child.props,
      children: child.children.map(mapNodeToComponent)
    }

    nativeRuntime.createView(JSON.stringify(componentData))
  },

  // Remove node
  remove(el: NativeNode): void {
    console.log(`Removing node ${el.id}`)
    
    if (el.parent) {
      const index = el.parent.children.indexOf(el)
      if (index > -1) {
        el.parent.children.splice(index, 1)
      }
    }
    
    // TODO: Remove from native layer
  },

  // Set props
  patchProp(
    el: NativeElement,
    key: string,
    prevValue: any,
    nextValue: any,
    isSVG?: boolean,
    prevChildren?: VNode[],
    parentComponent?: any,
    parentSuspense?: any,
    unmountChildren?: any
  ): void {
    console.log(`Setting prop ${key} = ${nextValue} on element ${el.id}`)
    
    // Handle special props
    switch (key) {
      case 'onClick':
      case 'onPress':
        el.props.onPress = nextValue
        break
      case 'style':
        // Convert Vue style to native props
        if (typeof nextValue === 'object') {
          Object.assign(el.props, convertStyleToNative(nextValue))
        }
        break
      default:
        el.props[key] = nextValue
    }

    // Update native view
    nativeRuntime.updateView(el.id, JSON.stringify(el.props))
  },

  // Parent node operations
  parentNode(node: NativeNode): NativeElement | null {
    return node.parent as NativeElement || null
  },

  // Next sibling
  nextSibling(node: NativeNode): NativeNode | null {
    if (!node.parent) return null
    
    const index = node.parent.children.indexOf(node)
    return node.parent.children[index + 1] || null
  },

  // Query selector (not really used in native)
  querySelector(): null {
    return null
  },

  // Set scope id (for scoped CSS)
  setScopeId(el: NativeElement, id: string): void {
    el.props.scopeId = id
  },

  // Clone node
  cloneNode(el: NativeNode): NativeNode {
    return {
      ...el,
      id: generateViewId(),
      children: [...el.children],
      parent: undefined
    }
  },

  // Insert static content (for optimization)
  insertStaticContent(): [NativeNode, NativeNode] {
    // Not implemented for native
    throw new Error('insertStaticContent not implemented')
  }
}

// Create the custom Vue renderer
export const renderer = createRenderer(rendererOptions)

// Create app function that uses our custom renderer
export function createNativeApp(rootComponent: any) {
  const app = renderer.createApp(rootComponent)
  
  // Initialize native runtime
  nativeRuntime.initialize()
  
  // Custom mount function
  const originalMount = app.mount
  app.mount = function(rootContainer?: any) {
    // Create root container
    const root: NativeElement = {
      id: 0,
      type: 'element',
      tag: 'View',
      props: {},
      children: []
    }
    
    return originalMount.call(this, root)
  }
  
  return app
}

// Utilities
let viewIdCounter = 1
function generateViewId(): number {
  return viewIdCounter++
}

// Map Vue tags to native view types
function mapVueTagToNative(tag: string): string {
  const tagMap: Record<string, string> = {
    'div': 'View',
    'span': 'Text',
    'p': 'Text',
    'button': 'Button',
    'input': 'TextInput',
    'img': 'Image',
    'text': 'Text',
    'view': 'View',
    'scroll-view': 'ScrollView'
  }
  
  return tagMap[tag.toLowerCase()] || 'View'
}

// Convert Vue component to native component data
function mapNodeToComponent(node: NativeNode): any {
  return {
    type: node.type === 'element' ? (node as NativeElement).tag : 'Text',
    props: node.props,
    children: node.children.map(mapNodeToComponent)
  }
}

// Convert Vue style object to native props
function convertStyleToNative(style: Record<string, any>): Record<string, any> {
  const nativeProps: Record<string, any> = {}
  
  // Convert CSS properties to native equivalents
  if (style.width) nativeProps.width = parseFloat(style.width)
  if (style.height) nativeProps.height = parseFloat(style.height)
  if (style.backgroundColor) nativeProps.backgroundColor = style.backgroundColor
  if (style.color) nativeProps.textColor = style.color
  if (style.fontSize) nativeProps.fontSize = parseFloat(style.fontSize)
  if (style.fontWeight) nativeProps.fontWeight = style.fontWeight
  if (style.padding) nativeProps.padding = parseFloat(style.padding)
  if (style.margin) nativeProps.margin = parseFloat(style.margin)
  
  return nativeProps
}

// Export for global usage
export { nativeRuntime }

// Register gesture handler
export function onGesture(type: string, callback: (event: any) => void) {
  // TODO: Register gesture handler with native runtime
  console.log(`Registering gesture handler for ${type}`)
}