use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "8" cy = "12" ></ circle > < line y2 = "6" y1 = "3" x1 = "3" x2 = "6" ></ line > < line x2 = "18" x1 = "21" y1 = "3" y2 = "6" ></ line > < line y2 = "18" y1 = "21" x2 = "6" x1 = "3" ></ line > < line x2 = "18" y1 = "21" x1 = "21" y2 = "18" ></ line > < / > } } pub const LucideCurrency : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;