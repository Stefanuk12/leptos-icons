use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" y = "3" ry = "2" width = "18" x = "3" ></ rect > < line x2 = "21" x1 = "3" y2 = "9" y1 = "9" ></ line > < line x1 = "3" y2 = "15" y1 = "15" x2 = "21" ></ line > < line y1 = "9" x1 = "9" x2 = "9" y2 = "21" ></ line > < line x1 = "15" y1 = "9" x2 = "15" y2 = "21" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor")] } ;