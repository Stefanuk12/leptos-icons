use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 19H5V13" ></ path > < path d = "M19 5L5 19" ></ path > < / > } } pub const LUCIDE_MOVE_DOWN_LEFT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;