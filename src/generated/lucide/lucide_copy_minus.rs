use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" x2 = "18" y1 = "15" y2 = "15" ></ line > < rect x = "8" ry = "2" width = "14" y = "8" height = "14" rx = "2" ></ rect > < path d = "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" ></ path > < / > } } pub const LUCIDE_COPY_MINUS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none")] } ;