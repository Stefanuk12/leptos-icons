use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "8" ></ circle > < line y2 = "6" x2 = "6" x1 = "3" y1 = "3" ></ line > < line y2 = "6" x1 = "21" x2 = "18" y1 = "3" ></ line > < line y2 = "18" x2 = "6" x1 = "3" y1 = "21" ></ line > < line y1 = "21" x1 = "21" y2 = "18" x2 = "18" ></ line > < / > } } pub const LUCIDE_CURRENCY : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;