use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 7v10" ></ path > < path d = "M6 5v14" ></ path > < rect x = "10" y = "3" height = "18" rx = "2" width = "12" ></ rect > < / > } } pub const LUCIDE_GALLERY_HORIZONTAL_END : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;