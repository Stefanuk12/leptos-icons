use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < line y2 = "15" y1 = "15" x1 = "8" x2 = "16" ></ line > < line x1 = "9" y1 = "9" y2 = "9" x2 = "9.01" ></ line > < line y1 = "9" x2 = "15.01" x1 = "15" y2 = "9" ></ line > < / > } } pub const LucideMeh : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;