use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 21h6a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v6" ></ path > < path d = "m3 21 9-9" ></ path > < path d = "M9 21H3v-6" ></ path > < / > } } pub const LUCIDE_SQUARE_ARROW_OUT_DOWN_LEFT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;