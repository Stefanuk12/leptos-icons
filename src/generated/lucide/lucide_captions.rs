use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" y = "5" height = "14" ry = "2" rx = "2" ></ rect > < path d = "M7 15h4M15 15h2M7 11h2M13 11h4" ></ path > < / > } } pub const LUCIDE_CAPTIONS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;