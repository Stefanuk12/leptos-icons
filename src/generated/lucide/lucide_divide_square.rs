use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" ry = "2" rx = "2" height = "18" x = "3" width = "18" ></ rect > < line y2 = "12" x2 = "16" x1 = "8" y1 = "12" ></ line > < line x2 = "12" x1 = "12" y2 = "16" y1 = "16" ></ line > < line x1 = "12" y2 = "8" y1 = "8" x2 = "12" ></ line > < / > } } pub const LucideDivideSquare : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;