use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 20h.01" ></ path > < path d = "M8.5 16.429a5 5 0 0 1 7 0" ></ path > < path d = "M5 12.859a10 10 0 0 1 5.17-2.69" ></ path > < path d = "M19 12.859a10 10 0 0 0-2.007-1.523" ></ path > < path d = "M2 8.82a15 15 0 0 1 4.177-2.643" ></ path > < path d = "M22 8.82a15 15 0 0 0-11.288-3.764" ></ path > < path d = "m2 2 20 20" ></ path > < / > } } pub const LUCIDE_WIFI_OFF : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;