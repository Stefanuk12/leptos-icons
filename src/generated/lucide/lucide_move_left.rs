use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 8L2 12L6 16" ></ path > < path d = "M2 12H22" ></ path > < / > } } pub const LUCIDE_MOVE_LEFT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;