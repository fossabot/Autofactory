import org.graalvm.nativeimage.c._
import org.graalvm.nativeimage.c.struct.{ CStruct, CField, CFieldAddress }
import org.graalvm.word.PointerBase
import org.graalvm.nativeimage.c.function.{ CFunction, CLibrary }

object Main:
    def main(args: Array[String]): Unit =
        print("Hi")

class myAnnotation extends annotation.Annotation
@CContext(classOf[Test.Headers])
@CLibrary("triple")
object Test:
    class Headers extends CContext.Directives:
        override def getHeaderFiles: java.util.List[String] = java.util.Collections.singletonList("\"test.h\"").nn
    @CStruct("value_t") trait Value extends PointerBase:
        @CField("type") var t: Int
        @CField("id") var id: Long
    @CStruct("triple_t") trait Triple extends PointerBase:
        @CFieldAddress("subject") def subject: Value
        @CFieldAddress("predicate") def predicate: Value
        @CFieldAddress("object") def obj: Value
    @CFunction(transition = CFunction.Transition.NO_TRANSITION)
    @native def allocRandomTriple: Triple