use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v20" ></ path > < path d = "m8 18 4 4 4-4" ></ path > < path d = "m8 6 4-4 4 4" ></ path > < / > } } pub const LUCIDE_MOVE_VERTICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;