use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2a10 10 0 0 1 7.38 16.75" ></ path > < path d = "m16 12-4-4-4 4" ></ path > < path d = "M12 16V8" ></ path > < path d = "M2.5 8.875a10 10 0 0 0-.5 3" ></ path > < path d = "M2.83 16a10 10 0 0 0 2.43 3.4" ></ path > < path d = "M4.636 5.235a10 10 0 0 1 .891-.857" ></ path > < path d = "M8.644 21.42a10 10 0 0 0 7.631-.38" ></ path > < / > } } pub const LUCIDE_CIRCLE_FADING_ARROW_UP : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor")] } ;