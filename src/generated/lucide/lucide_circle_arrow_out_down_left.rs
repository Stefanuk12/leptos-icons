use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 12a10 10 0 1 1 10 10" ></ path > < path d = "m2 22 10-10" ></ path > < path d = "M8 22H2v-6" ></ path > < / > } } pub const LUCIDE_CIRCLE_ARROW_OUT_DOWN_LEFT : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;