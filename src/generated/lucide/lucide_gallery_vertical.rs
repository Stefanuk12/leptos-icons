use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 2h18" ></ path > < rect x = "3" height = "12" y = "6" width = "18" rx = "2" ></ rect > < path d = "M3 22h18" ></ path > < / > } } pub const LUCIDE_GALLERY_VERTICAL : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;