use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" x1 = "8" y2 = "12" x2 = "16" ></ line > < line y2 = "16" x2 = "12" y1 = "16" x1 = "12" ></ line > < line x1 = "12" y2 = "8" x2 = "12" y1 = "8" ></ line > < circle cx = "12" cy = "12" r = "10" ></ circle > < / > } } pub const LucideDivideCircle : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;