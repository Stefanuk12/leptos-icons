use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "18" x1 = "12" x2 = "18" y1 = "12" ></ line > < line x1 = "12" y2 = "12" x2 = "18" y1 = "18" ></ line > < rect y = "8" rx = "2" ry = "2" width = "14" x = "8" height = "14" ></ rect > < path d = "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" ></ path > < / > } } pub const LUCIDE_COPY_X : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;