use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "8" cx = "12" ></ circle > < line x1 = "3" y1 = "3" y2 = "6" x2 = "6" ></ line > < line x2 = "18" x1 = "21" y1 = "3" y2 = "6" ></ line > < line y2 = "18" x1 = "3" x2 = "6" y1 = "21" ></ line > < line x2 = "18" y1 = "21" x1 = "21" y2 = "18" ></ line > < / > } } pub const LUCIDE_CURRENCY : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24")] } ;