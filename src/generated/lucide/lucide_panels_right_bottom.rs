use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" x = "3" y = "3" rx = "2" ></ rect > < path d = "M3 15h12" ></ path > < path d = "M15 3v18" ></ path > < / > } } pub const LUCIDE_PANELS_RIGHT_BOTTOM : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;