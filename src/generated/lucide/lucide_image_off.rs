use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "22" x2 = "22" y1 = "2" x1 = "2" ></ line > < path d = "M10.41 10.41a2 2 0 1 1-2.83-2.83" ></ path > < line x2 = "6" y1 = "13.5" x1 = "13.5" y2 = "21" ></ line > < line y1 = "12" x2 = "21" x1 = "18" y2 = "15" ></ line > < path d = "M3.59 3.59A1.99 1.99 0 0 0 3 5v14a2 2 0 0 0 2 2h14c.55 0 1.052-.22 1.41-.59" ></ path > < path d = "M21 15V5a2 2 0 0 0-2-2H9" ></ path > < / > } } pub const LUCIDE_IMAGE_OFF : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;