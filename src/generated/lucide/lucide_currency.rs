use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" cx = "12" r = "8" ></ circle > < line x1 = "3" y1 = "3" x2 = "6" y2 = "6" ></ line > < line y2 = "6" y1 = "3" x2 = "18" x1 = "21" ></ line > < line x2 = "6" y2 = "18" x1 = "3" y1 = "21" ></ line > < line x1 = "21" y2 = "18" y1 = "21" x2 = "18" ></ line > < / > } } pub const LUCIDE_CURRENCY : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none")] } ;