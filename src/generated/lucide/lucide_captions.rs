use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" y = "5" height = "14" x = "3" rx = "2" width = "18" ></ rect > < path d = "M7 15h4M15 15h2M7 11h2M13 11h4" ></ path > < / > } } pub const LUCIDE_CAPTIONS : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;