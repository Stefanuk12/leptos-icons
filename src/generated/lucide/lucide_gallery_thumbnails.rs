use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" width = "18" x = "3" y = "3" rx = "2" ></ rect > < path d = "M4 21h1" ></ path > < path d = "M9 21h1" ></ path > < path d = "M14 21h1" ></ path > < path d = "M19 21h1" ></ path > < / > } } pub const LUCIDE_GALLERY_THUMBNAILS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;