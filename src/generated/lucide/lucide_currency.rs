use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "8" cx = "12" cy = "12" ></ circle > < line x1 = "3" x2 = "6" y1 = "3" y2 = "6" ></ line > < line y2 = "6" y1 = "3" x2 = "18" x1 = "21" ></ line > < line y2 = "18" x2 = "6" y1 = "21" x1 = "3" ></ line > < line x2 = "18" x1 = "21" y2 = "18" y1 = "21" ></ line > < / > } } pub const LUCIDE_CURRENCY : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;