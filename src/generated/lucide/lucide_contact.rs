use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect height = "18" y = "4" x = "3" width = "18" rx = "2" ></ rect > < circle r = "2" cx = "12" cy = "10" ></ circle > < line x2 = "8" y2 = "4" y1 = "2" x1 = "8" ></ line > < line y2 = "4" y1 = "2" x2 = "16" x1 = "16" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;