use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < line x1 = "8" y2 = "15" y1 = "15" x2 = "16" ></ line > < line y2 = "9" x1 = "9" x2 = "9.01" y1 = "9" ></ line > < line y1 = "9" x1 = "15" y2 = "9" x2 = "15.01" ></ line > < / > } } pub const LucideMeh : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;