use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 6H6v8h8V6Z" ></ path > < path fill - rule = "evenodd" d = "M9.25 3V1.75a.75.75 0 0 1 1.5 0V3h1.5V1.75a.75.75 0 0 1 1.5 0V3h.5A2.75 2.75 0 0 1 17 5.75v.5h1.25a.75.75 0 0 1 0 1.5H17v1.5h1.25a.75.75 0 0 1 0 1.5H17v1.5h1.25a.75.75 0 0 1 0 1.5H17v.5A2.75 2.75 0 0 1 14.25 17h-.5v1.25a.75.75 0 0 1-1.5 0V17h-1.5v1.25a.75.75 0 0 1-1.5 0V17h-1.5v1.25a.75.75 0 0 1-1.5 0V17h-.5A2.75 2.75 0 0 1 3 14.25v-.5H1.75a.75.75 0 0 1 0-1.5H3v-1.5H1.75a.75.75 0 0 1 0-1.5H3v-1.5H1.75a.75.75 0 0 1 0-1.5H3v-.5A2.75 2.75 0 0 1 5.75 3h.5V1.75a.75.75 0 0 1 1.5 0V3h1.5ZM4.5 5.75c0-.69.56-1.25 1.25-1.25h8.5c.69 0 1.25.56 1.25 1.25v8.5c0 .69-.56 1.25-1.25 1.25h-8.5c-.69 0-1.25-.56-1.25-1.25v-8.5Z" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_CPU_CHIP : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("fill" , "currentColor") , ("viewBox" , "0 0 20 20") , ("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true")] } ;