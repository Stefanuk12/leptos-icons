use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 11V5H11" ></ path > < path d = "M5 5L19 19" ></ path > < / > } } pub const LUCIDE_MOVE_UP_LEFT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;