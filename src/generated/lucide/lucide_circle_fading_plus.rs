use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2a10 10 0 0 1 7.38 16.75" ></ path > < path d = "M12 8v8" ></ path > < path d = "M16 12H8" ></ path > < path d = "M2.5 8.875a10 10 0 0 0-.5 3" ></ path > < path d = "M2.83 16a10 10 0 0 0 2.43 3.4" ></ path > < path d = "M4.636 5.235a10 10 0 0 1 .891-.857" ></ path > < path d = "M8.644 21.42a10 10 0 0 0 7.631-.38" ></ path > < / > } } pub const LUCIDE_CIRCLE_FADING_PLUS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;