use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" width = "18" y = "5" x = "3" rx = "2" height = "14" ></ rect > < path d = "M7 15h4M15 15h2M7 11h2M13 11h4" ></ path > < / > } } pub const LUCIDE_CAPTIONS : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;