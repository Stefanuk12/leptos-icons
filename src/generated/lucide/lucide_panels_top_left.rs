use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" x = "3" width = "18" height = "18" ></ rect > < path d = "M3 9h18" ></ path > < path d = "M9 21V9" ></ path > < / > } } pub const LUCIDE_PANELS_TOP_LEFT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;