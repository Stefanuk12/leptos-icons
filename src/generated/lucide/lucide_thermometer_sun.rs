use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 9a4 4 0 0 0-2 7.5" ></ path > < path d = "M12 3v2" ></ path > < path d = "m6.6 18.4-1.4 1.4" ></ path > < path d = "M20 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z" ></ path > < path d = "M4 13H2" ></ path > < path d = "M6.34 7.34 4.93 5.93" ></ path > < / > } } pub const LUCIDE_THERMOMETER_SUN : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor")] } ;