use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" ry = "2" rx = "2" height = "14" y = "5" ></ rect > < path d = "M7 15h4M15 15h2M7 11h2M13 11h4" ></ path > < / > } } pub const LUCIDE_CAPTIONS : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round")] } ;