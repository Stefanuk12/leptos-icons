use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" y = "3" rx = "2" height = "14" ></ rect > < path d = "M4 21h1" ></ path > < path d = "M9 21h1" ></ path > < path d = "M14 21h1" ></ path > < path d = "M19 21h1" ></ path > < / > } } pub const LUCIDE_GALLERY_THUMBNAILS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none")] } ;