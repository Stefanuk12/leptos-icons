use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 2h18" ></ path > < rect x = "3" y = "6" rx = "2" width = "18" height = "12" ></ rect > < path d = "M3 22h18" ></ path > < / > } } pub const LUCIDE_GALLERY_VERTICAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;