use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 5H6a2 2 0 0 0-2 2v3" ></ path > < path d = "m9 8 3-3-3-3" ></ path > < path d = "M4 14v4a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2" ></ path > < / > } } pub const LUCIDE_ROTATE_CW_SQUARE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2")] } ;