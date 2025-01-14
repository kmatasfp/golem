// Generated by `wit-bindgen` 0.37.0. DO NOT EDIT!
package wit.worlds;

import java.nio.charset.StandardCharsets;
import java.util.ArrayList;

import org.teavm.interop.Memory;
import org.teavm.interop.Address;
import org.teavm.interop.Import;
import org.teavm.interop.Export;
import org.teavm.interop.CustomSection;

public final class JavaExample{
    private JavaExample() {}
    
    @Export(name = "run-example1")
    private static int wasmExportRunExample1(int p0, int p1) {
        
        byte[] bytes = new byte[p1];
        Memory.getBytes(org.teavm.interop.Address.fromInt(p0), bytes, 0, p1);
        
        int result = wit.worlds.JavaExampleImpl.runExample1(new String(bytes, StandardCharsets.UTF_8));
        
        return result;
        
    }
    
    @CustomSection(name = "component-type:JavaExample")
    private static final String __WIT_BINDGEN_COMPONENT_TYPE = "0061736d0d0001000019167769742d636f6d706f6e656e742d656e636f64696e6704000739014102014102014001016173007904000c72756e2d6578616d706c65310100040015676f6c656d3a69742f6a6176612d6578616d706c6504000b1201000c6a6176612d6578616d706c65030000004d0970726f647563657273010c70726f6365737365642d6279020d7769742d636f6d706f6e656e7407302e3232332e30167769742d62696e6467656e2d746561766d2d6a61766106302e33372e30";
    
}
