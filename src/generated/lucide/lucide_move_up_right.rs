use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 5H19V11" ></ path > < path d = "M19 5L5 19" ></ path > < / > } } pub const LUCIDE_MOVE_UP_RIGHT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round")] } ;