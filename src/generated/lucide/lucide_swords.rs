use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "14.5 17.5 3 6 3 3 6 3 17.5 14.5" ></ polyline > < line x1 = "13" y1 = "19" x2 = "19" y2 = "13" ></ line > < line x1 = "16" x2 = "20" y1 = "16" y2 = "20" ></ line > < line y1 = "21" x2 = "21" y2 = "19" x1 = "19" ></ line > < polyline points = "14.5 6.5 18 3 21 3 21 6 17.5 9.5" ></ polyline > < line x2 = "9" y2 = "18" x1 = "5" y1 = "14" ></ line > < line y1 = "17" y2 = "20" x1 = "7" x2 = "4" ></ line > < line x1 = "3" y2 = "21" y1 = "19" x2 = "5" ></ line > < / > } } pub const LucideSwords : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2")] } ;