use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" rx = "2" y = "3" ry = "2" x = "3" ></ rect > < line x1 = "3" x2 = "21" y2 = "9" y1 = "9" ></ line > < line y2 = "15" x2 = "21" y1 = "15" x1 = "3" ></ line > < line x2 = "9" y2 = "21" y1 = "9" x1 = "9" ></ line > < line x2 = "15" y1 = "9" x1 = "15" y2 = "21" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;