use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" y = "3" width = "18" height = "18" rx = "2" x = "3" ></ rect > < line x1 = "3" y1 = "9" y2 = "9" x2 = "21" ></ line > < line x1 = "3" x2 = "21" y1 = "15" y2 = "15" ></ line > < line x1 = "9" y1 = "9" x2 = "9" y2 = "21" ></ line > < line x1 = "15" y2 = "21" y1 = "9" x2 = "15" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;