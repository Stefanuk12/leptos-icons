use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 19H5V13" ></ path > < path d = "M19 5L5 19" ></ path > < / > } } pub const LUCIDE_MOVE_DOWN_LEFT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;