use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "18" x1 = "12" y1 = "18" y2 = "12" ></ line > < rect width = "14" height = "14" x = "8" y = "8" rx = "2" ry = "2" ></ rect > < path d = "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" ></ path > < / > } } pub const LUCIDE_COPY_SLASH : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;