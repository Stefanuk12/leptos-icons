use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cy = "5" cx = "18" ></ circle > < circle r = "3" cx = "6" cy = "12" ></ circle > < circle r = "3" cy = "19" cx = "18" ></ circle > < line y1 = "13.51" x1 = "8.59" y2 = "17.49" x2 = "15.42" ></ line > < line y1 = "6.51" x1 = "15.41" x2 = "8.59" y2 = "10.49" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;