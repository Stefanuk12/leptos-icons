use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 11V5H11" ></ path > < path d = "M5 5L19 19" ></ path > < / > } } pub const LUCIDE_MOVE_UP_LEFT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none")] } ;