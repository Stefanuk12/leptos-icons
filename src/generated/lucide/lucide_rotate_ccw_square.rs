use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 9V7a2 2 0 0 0-2-2h-6" ></ path > < path d = "m15 2-3 3 3 3" ></ path > < path d = "M20 13v5a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2" ></ path > < / > } } pub const LUCIDE_ROTATE_CCW_SQUARE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24")] } ;