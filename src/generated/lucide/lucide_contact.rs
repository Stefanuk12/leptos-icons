use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect height = "18" x = "3" width = "18" rx = "2" y = "4" ></ rect > < circle cx = "12" r = "2" cy = "10" ></ circle > < line y2 = "4" y1 = "2" x1 = "8" x2 = "8" ></ line > < line y1 = "2" x1 = "16" y2 = "4" x2 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;