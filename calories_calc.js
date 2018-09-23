(function() {
    var wasm;
    const __exports = {};


    const __widl_f_import_node_with_deep_Document_target = Document.prototype.importNode || function() {
        throw new Error(`wasm-bindgen: Document.prototype.importNode does not exist`);
    };

    let cachegetUint32Memory = null;
    function getUint32Memory() {
        if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
            cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
        }
        return cachegetUint32Memory;
    }

    const slab = [{ obj: undefined }, { obj: null }, { obj: true }, { obj: false }];

    let slab_next = slab.length;

    function addHeapObject(obj) {
        if (slab_next === slab.length) slab.push(slab.length + 1);
        const idx = slab_next;
        const next = slab[idx];

        slab_next = next;

        slab[idx] = { obj, cnt: 1 };
        return idx << 1;
    }

    const stack = [];

    function getObject(idx) {
        if ((idx & 1) === 1) {
            return stack[idx >> 1];
        } else {
            const val = slab[idx >> 1];

            return val.obj;

        }
    }

    __exports.__widl_f_import_node_with_deep_Document = function(arg0, arg1, arg2, exnptr) {
        try {
            return addHeapObject(__widl_f_import_node_with_deep_Document_target.call(getObject(arg0), getObject(arg1), arg2 !== 0));
        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    const __widl_f_query_selector_Document_target = Document.prototype.querySelector || function() {
        throw new Error(`wasm-bindgen: Document.prototype.querySelector does not exist`);
    };

    let cachedDecoder = new TextDecoder('utf-8');

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    function getStringFromWasm(ptr, len) {
        return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    __exports.__widl_f_query_selector_Document = function(arg0, arg1, arg2, exnptr) {
        let varg1 = getStringFromWasm(arg1, arg2);
        try {

            const val = __widl_f_query_selector_Document_target.call(getObject(arg0), varg1);
            return isLikeNone(val) ? 0 : addHeapObject(val);

        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    function GetOwnOrInheritedPropertyDescriptor(obj, id) {
        while (obj) {
            let desc = Object.getOwnPropertyDescriptor(obj, id);
            if (desc) return desc;
            obj = Object.getPrototypeOf(obj);
        }
        throw new Error(`descriptor for id='${id}' not found`);
    }

    const __widl_f_document_element_Document_target = GetOwnOrInheritedPropertyDescriptor(Document.prototype, 'documentElement').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Document.prototype, 'documentElement').get does not exist`);
    };

    __exports.__widl_f_document_element_Document = function(arg0) {

        const val = __widl_f_document_element_Document_target.call(getObject(arg0));
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };

    const __widl_f_ready_state_Document_target = GetOwnOrInheritedPropertyDescriptor(Document.prototype, 'readyState').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Document.prototype, 'readyState').get does not exist`);
    };

    let cachedEncoder = new TextEncoder('utf-8');

    function passStringToWasm(arg) {

        const buf = cachedEncoder.encode(arg);
        const ptr = wasm.__wbindgen_malloc(buf.length);
        getUint8Memory().set(buf, ptr);
        return [ptr, buf.length];
    }

    __exports.__widl_f_ready_state_Document = function(ret, arg0) {

        const [retptr, retlen] = passStringToWasm(__widl_f_ready_state_Document_target.call(getObject(arg0)));
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };

    const __widl_f_query_selector_DocumentFragment_target = DocumentFragment.prototype.querySelector || function() {
        throw new Error(`wasm-bindgen: DocumentFragment.prototype.querySelector does not exist`);
    };

    __exports.__widl_f_query_selector_DocumentFragment = function(arg0, arg1, arg2, exnptr) {
        let varg1 = getStringFromWasm(arg1, arg2);
        try {

            const val = __widl_f_query_selector_DocumentFragment_target.call(getObject(arg0), varg1);
            return isLikeNone(val) ? 0 : addHeapObject(val);

        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    const __widl_f_closest_Element_target = Element.prototype.closest || function() {
        throw new Error(`wasm-bindgen: Element.prototype.closest does not exist`);
    };

    __exports.__widl_f_closest_Element = function(arg0, arg1, arg2, exnptr) {
        let varg1 = getStringFromWasm(arg1, arg2);
        try {

            const val = __widl_f_closest_Element_target.call(getObject(arg0), varg1);
            return isLikeNone(val) ? 0 : addHeapObject(val);

        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    const __widl_f_query_selector_Element_target = Element.prototype.querySelector || function() {
        throw new Error(`wasm-bindgen: Element.prototype.querySelector does not exist`);
    };

    __exports.__widl_f_query_selector_Element = function(arg0, arg1, arg2, exnptr) {
        let varg1 = getStringFromWasm(arg1, arg2);
        try {

            const val = __widl_f_query_selector_Element_target.call(getObject(arg0), varg1);
            return isLikeNone(val) ? 0 : addHeapObject(val);

        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    const __widl_f_id_Element_target = GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'id').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'id').get does not exist`);
    };

    __exports.__widl_f_id_Element = function(ret, arg0) {

        const [retptr, retlen] = passStringToWasm(__widl_f_id_Element_target.call(getObject(arg0)));
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };

    const __widl_f_class_name_Element_target = GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'className').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'className').get does not exist`);
    };

    __exports.__widl_f_class_name_Element = function(ret, arg0) {

        const [retptr, retlen] = passStringToWasm(__widl_f_class_name_Element_target.call(getObject(arg0)));
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };

    const __widl_f_set_class_name_Element_target = GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'className').set || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Element.prototype, 'className').set does not exist`);
    };

    __exports.__widl_f_set_class_name_Element = function(arg0, arg1, arg2) {
        let varg1 = getStringFromWasm(arg1, arg2);
        __widl_f_set_class_name_Element_target.call(getObject(arg0), varg1);
    };

    const __widl_f_remove_Element_target = Element.prototype.remove || function() {
        throw new Error(`wasm-bindgen: Element.prototype.remove does not exist`);
    };

    __exports.__widl_f_remove_Element = function(arg0) {
        __widl_f_remove_Element_target.call(getObject(arg0));
    };

    const __widl_f_target_Event_target = GetOwnOrInheritedPropertyDescriptor(Event.prototype, 'target').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Event.prototype, 'target').get does not exist`);
    };

    __exports.__widl_f_target_Event = function(arg0) {

        const val = __widl_f_target_Event_target.call(getObject(arg0));
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };

    const __widl_f_add_event_listener_with_callback_EventTarget_target = EventTarget.prototype.addEventListener || function() {
        throw new Error(`wasm-bindgen: EventTarget.prototype.addEventListener does not exist`);
    };

    __exports.__widl_f_add_event_listener_with_callback_EventTarget = function(arg0, arg1, arg2, arg3, exnptr) {
        let varg1 = getStringFromWasm(arg1, arg2);
        try {
            __widl_f_add_event_listener_with_callback_EventTarget_target.call(getObject(arg0), varg1, getObject(arg3));
        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    const __widl_f_focus_HTMLElement_target = HTMLElement.prototype.focus || function() {
        throw new Error(`wasm-bindgen: HTMLElement.prototype.focus does not exist`);
    };

    __exports.__widl_f_focus_HTMLElement = function(arg0, exnptr) {
        try {
            __widl_f_focus_HTMLElement_target.call(getObject(arg0));
        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    const __widl_f_value_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLInputElement.prototype, 'value').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLInputElement.prototype, 'value').get does not exist`);
    };

    __exports.__widl_f_value_HTMLInputElement = function(ret, arg0) {

        const [retptr, retlen] = passStringToWasm(__widl_f_value_HTMLInputElement_target.call(getObject(arg0)));
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };

    const __widl_f_set_value_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLInputElement.prototype, 'value').set || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLInputElement.prototype, 'value').set does not exist`);
    };

    __exports.__widl_f_set_value_HTMLInputElement = function(arg0, arg1, arg2) {
        let varg1 = getStringFromWasm(arg1, arg2);
        __widl_f_set_value_HTMLInputElement_target.call(getObject(arg0), varg1);
    };

    const __widl_f_content_HTMLTemplateElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLTemplateElement.prototype, 'content').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLTemplateElement.prototype, 'content').get does not exist`);
    };

    __exports.__widl_f_content_HTMLTemplateElement = function(arg0) {
        return addHeapObject(__widl_f_content_HTMLTemplateElement_target.call(getObject(arg0)));
    };

    const __widl_f_key_KeyboardEvent_target = GetOwnOrInheritedPropertyDescriptor(KeyboardEvent.prototype, 'key').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(KeyboardEvent.prototype, 'key').get does not exist`);
    };

    __exports.__widl_f_key_KeyboardEvent = function(ret, arg0) {

        const [retptr, retlen] = passStringToWasm(__widl_f_key_KeyboardEvent_target.call(getObject(arg0)));
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };

    const __widl_f_insert_before_Node_target = Node.prototype.insertBefore || function() {
        throw new Error(`wasm-bindgen: Node.prototype.insertBefore does not exist`);
    };

    __exports.__widl_f_insert_before_Node = function(arg0, arg1, arg2, exnptr) {
        try {
            return addHeapObject(__widl_f_insert_before_Node_target.call(getObject(arg0), getObject(arg1), getObject(arg2)));
        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    const __widl_f_first_child_Node_target = GetOwnOrInheritedPropertyDescriptor(Node.prototype, 'firstChild').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Node.prototype, 'firstChild').get does not exist`);
    };

    __exports.__widl_f_first_child_Node = function(arg0) {

        const val = __widl_f_first_child_Node_target.call(getObject(arg0));
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };

    const __widl_f_text_content_Node_target = GetOwnOrInheritedPropertyDescriptor(Node.prototype, 'textContent').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Node.prototype, 'textContent').get does not exist`);
    };

    __exports.__widl_f_text_content_Node = function(ret, arg0) {
        const val = __widl_f_text_content_Node_target.call(getObject(arg0));
        const [retptr, retlen] = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };

    const __widl_f_set_text_content_Node_target = GetOwnOrInheritedPropertyDescriptor(Node.prototype, 'textContent').set || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Node.prototype, 'textContent').set does not exist`);
    };

    __exports.__widl_f_set_text_content_Node = function(arg0, arg1, arg2) {
        let varg1 = arg1 == 0 ? undefined : getStringFromWasm(arg1, arg2);
        __widl_f_set_text_content_Node_target.call(getObject(arg0), varg1);
    };

    __exports.__widl_instanceof_Window = function(idx) {
        return getObject(idx) instanceof Window ? 1 : 0;
    };

    const __widl_f_document_Window_target = function() {
        return this.document;
    };

    __exports.__widl_f_document_Window = function(arg0) {

        const val = __widl_f_document_Window_target.call(getObject(arg0));
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };

    __exports.__wbg_newnoargs_f73713b21933b352 = function(arg0, arg1) {
        let varg0 = getStringFromWasm(arg0, arg1);
        return addHeapObject(new Function(varg0));
    };

    const __wbg_call_7b63ecfafcd67469_target = Function.prototype.call || function() {
        throw new Error(`wasm-bindgen: Function.prototype.call does not exist`);
    };

    __exports.__wbg_call_7b63ecfafcd67469 = function(arg0, arg1, exnptr) {
        try {
            return addHeapObject(__wbg_call_7b63ecfafcd67469_target.call(getObject(arg0), getObject(arg1)));
        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };
    /**
    * @returns {void}
    */
    __exports.init = function() {
        return wasm.init();
    };

    __exports.__wbindgen_object_clone_ref = function(idx) {
        // If this object is on the stack promote it to the heap.
        if ((idx & 1) === 1) return addHeapObject(getObject(idx));

        // Otherwise if the object is on the heap just bump the
        // refcount and move on
        const val = slab[idx >> 1];
        val.cnt += 1;
        return idx;
    };

    function dropRef(idx) {

        idx = idx >> 1;
        if (idx < 4) return;
        let obj = slab[idx];

        obj.cnt -= 1;
        if (obj.cnt > 0) return;

        // If we hit 0 then free up our space in the slab
        slab[idx] = slab_next;
        slab_next = idx;
    }

    __exports.__wbindgen_object_drop_ref = function(i) {
        dropRef(i);
    };

    __exports.__wbindgen_cb_forget = function(i) {
        dropRef(i);
    };

    function takeObject(idx) {
        const ret = getObject(idx);
        dropRef(idx);
        return ret;
    }

    __exports.__wbindgen_rethrow = function(idx) { throw takeObject(idx); };

    __exports.__wbindgen_closure_wrapper382 = function(ptr, f, _ignored) {
        let cb = function(arg0) {
            let a = this.a;
            this.a = 0;
            try {
                return this.f(a, addHeapObject(arg0));

            } finally {
                this.a = a;

            }

        };
        cb.f = wasm.__wbg_function_table.get(f);
        cb.a = ptr;
        let real = cb.bind(cb);
        real.original = cb;
        return addHeapObject(real);
    };

    __exports.__wbindgen_throw = function(ptr, len) {
        throw new Error(getStringFromWasm(ptr, len));
    };

    function init(wasm_path) {
        const fetchPromise = fetch(wasm_path);
        let resultPromise;
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            resultPromise = WebAssembly.instantiateStreaming(fetchPromise, { './calories_calc': __exports });
        } else {
            resultPromise = fetchPromise
            .then(response => response.arrayBuffer())
            .then(buffer => WebAssembly.instantiate(buffer, { './calories_calc': __exports }));
        }
        return resultPromise.then(({instance}) => {
            wasm = init.wasm = instance.exports;
            return;
        });
    };
    self.wasm_bindgen = Object.assign(init, __exports);
})();
