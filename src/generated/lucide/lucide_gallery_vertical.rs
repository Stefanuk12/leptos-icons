use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 2h18" ></ path > < rect width = "18" x = "3" y = "6" height = "12" rx = "2" ></ rect > < path d = "M3 22h18" ></ path > < / > } } pub const LUCIDE_GALLERY_VERTICAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;