use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" x = "3" width = "18" height = "18" ></ rect > < path d = "M3 9h18" ></ path > < path d = "M9 21V9" ></ path > < / > } } pub const LUCIDE_PANELS_TOP_LEFT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor")] } ;