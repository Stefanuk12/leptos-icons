use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17 18a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2" ></ path > < rect width = "18" y = "4" height = "18" x = "3" rx = "2" ></ rect > < circle cy = "10" r = "2" cx = "12" ></ circle > < line y2 = "4" x1 = "8" x2 = "8" y1 = "2" ></ line > < line x1 = "16" y1 = "2" x2 = "16" y2 = "4" ></ line > < / > } } pub const LUCIDE_CONTACT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;