use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" rx = "2" height = "14" width = "18" ></ rect > < path d = "M4 21h1" ></ path > < path d = "M9 21h1" ></ path > < path d = "M14 21h1" ></ path > < path d = "M19 21h1" ></ path > < / > } } pub const LUCIDE_GALLERY_THUMBNAILS : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none")] } ;