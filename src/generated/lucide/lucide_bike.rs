use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18.5" cy = "17.5" r = "3.5" ></ circle > < circle r = "3.5" cx = "5.5" cy = "17.5" ></ circle > < circle r = "1" cx = "15" cy = "5" ></ circle > < path d = "M12 17.5V14l-3-3 4-3 2 3h2" ></ path > < / > } } pub const LUCIDE_BIKE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;